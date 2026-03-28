//! Custom state management module to replace bounce
//!
//! This module provides similar functionality to bounce using yew's built-in
//! context and reducer hooks. It supports three patterns:
//! - **Atom**: Simple global state with get/set semantics
//! - **Slice**: Reducer-based state with action dispatch
//! - **Selector**: Derived/computed state from other atoms

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

/// Trait for atom state (simple global state)
pub trait Atom: Clone + PartialEq + Default + 'static {}

/// Trait for slice state (with reducer pattern)
pub trait Slice: Clone + PartialEq + Default + 'static {
  type Action;
  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self>;
}

/// Trait for selector (derived state)
pub trait Selector: Clone + PartialEq + 'static {
  fn select(states: &BounceStates) -> Rc<Self>;
}

/// Type alias for subscriber callback
type SubscriberCallback = Rc<RefCell<dyn Fn()>>;

/// Context for accessing all atom states
#[derive(Clone)]
pub struct BounceStates {
  /// Type-erased storage keyed by TypeId for O(1) lookup
  store: Rc<RefCell<HashMap<TypeId, Rc<dyn Any>>>>,
  /// Subscribers notified on any state change
  subscribers: Rc<RefCell<Vec<SubscriberCallback>>>,
}

impl PartialEq for BounceStates {
  fn eq(&self, _other: &Self) -> bool {
    // Always return true since we use interior mutability
    // This is needed for ContextProvider
    true
  }
}

impl Default for BounceStates {
  fn default() -> Self {
    Self {
      store: Rc::new(RefCell::new(HashMap::new())),
      subscribers: Rc::new(RefCell::new(Vec::new())),
    }
  }
}

impl BounceStates {
  /// Create a new empty state container
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self::default()
  }

  /// Get a value from the store by type, returning default if not present
  fn get_value<T: Clone + Default + 'static>(&self) -> Rc<T> {
    let type_id = TypeId::of::<T>();
    let store = self.store.borrow();

    if let Some(value) = store.get(&type_id) {
      if let Ok(typed) = value.clone().downcast::<T>() {
        return typed;
      }
    }

    Rc::new(T::default())
  }

  /// Set a value in the store by type and notify subscribers
  fn set_value<T: 'static>(&self, value: Rc<T>) {
    let type_id = TypeId::of::<T>();
    self.store.borrow_mut().insert(type_id, value);
    self.notify_subscribers();
  }

  /// Get atom value from storage
  pub fn get_atom_value<T: Atom + 'static>(&self) -> Rc<T> {
    self.get_value::<T>()
  }

  /// Set atom value and notify subscribers
  pub fn set_atom_value<T: Atom + 'static>(&self, value: Rc<T>) {
    self.set_value::<T>(value);
  }

  /// Get slice value from storage
  pub fn get_slice_value<T: Slice + 'static>(&self) -> Rc<T> {
    self.get_value::<T>()
  }

  /// Dispatch slice action: read current -> reduce -> store new -> notify
  pub fn dispatch_slice_action<T: Slice + 'static>(&self, action: <T as Slice>::Action) {
    let current = self.get_slice_value::<T>();
    let new_value = current.reduce(action);
    self.set_value::<T>(new_value);
  }

  /// Subscribe to state changes, returns a Subscription handle that
  /// automatically unsubscribes on drop
  pub fn subscribe<F: Fn() + 'static>(&self, callback: F) -> Subscription {
    let callback: SubscriberCallback = Rc::new(RefCell::new(callback));
    self.subscribers.borrow_mut().push(callback.clone());
    Subscription {
      subscribers: self.subscribers.clone(),
      callback,
    }
  }

  /// Notify all subscribers of state changes
  fn notify_subscribers(&self) {
    // Clone the list to avoid borrow conflicts during callback execution
    let subscribers: Vec<SubscriberCallback> = self.subscribers.borrow().iter().cloned().collect();

    for callback in &subscribers {
      (callback.borrow())();
    }
  }
}

/// Subscription handle that removes the callback when dropped (RAII pattern)
pub struct Subscription {
  subscribers: Rc<RefCell<Vec<SubscriberCallback>>>,
  callback: SubscriberCallback,
}

impl Drop for Subscription {
  fn drop(&mut self) {
    self
      .subscribers
      .borrow_mut()
      .retain(|cb| !Rc::ptr_eq(cb, &self.callback));
  }
}

/// Props for BounceRoot component
#[derive(Properties, PartialEq)]
pub struct BounceRootProps {
  #[prop_or_default]
  pub children: Children,
}

/// Root component that provides state context
#[function_component]
pub fn BounceRoot(props: &BounceRootProps) -> Html {
  let states = use_state(BounceStates::default);

  html! {
      <ContextProvider<BounceStates> context={(*states).clone()}>
          {props.children.clone()}
      </ContextProvider<BounceStates>>
  }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Action for forcing component update
#[derive(Clone, PartialEq)]
struct ForceUpdateAction;

/// Counter that triggers re-render when incremented
#[derive(Clone, PartialEq, Default)]
struct UpdateCounter(u32);

impl Reducible for UpdateCounter {
  type Action = ForceUpdateAction;

  fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
    Rc::new(UpdateCounter(self.0.wrapping_add(1)))
  }
}

/// Hook helper: subscribe to BounceStates changes and force re-render.
/// Must be called from within a `#[hook]` function.
#[hook]
fn use_state_subscription(states: BounceStates) {
  let update_counter = use_reducer_eq(UpdateCounter::default);
  {
    let update_counter = update_counter.clone();
    let states_clone = states.clone();
    use_effect_with((), move |_| {
      let subscription = states_clone.subscribe(move || {
        update_counter.dispatch(ForceUpdateAction);
      });
      move || drop(subscription)
    });
  }
}

// ---------------------------------------------------------------------------
// Atom hooks
// ---------------------------------------------------------------------------

/// Hook to get atom value (subscribes to changes for re-render)
#[hook]
pub fn use_atom_value<T: Atom + 'static>() -> Rc<T> {
  let states = use_context::<BounceStates>().expect("BounceRoot not found");
  // Subscribe so the component re-renders when any state changes
  use_state_subscription(states.clone());
  states.get_atom_value::<T>()
}

/// Hook to get atom setter
#[hook]
pub fn use_atom_setter<T: Atom + 'static>() -> Callback<T> {
  let states = use_context::<BounceStates>().expect("BounceRoot not found");

  Callback::from(move |value: T| {
    states.set_atom_value(Rc::new(value));
  })
}

/// Hook to get atom value and setter
#[hook]
pub fn use_atom<T: Atom + 'static>() -> (Rc<T>, Callback<T>) {
  let value = use_atom_value::<T>();
  let setter = use_atom_setter::<T>();
  (value, setter)
}

// ---------------------------------------------------------------------------
// Slice hooks
// ---------------------------------------------------------------------------

/// Handle for slice state providing value access and action dispatch
pub struct SliceHandle<T: Slice> {
  /// Current slice value
  pub value: Rc<T>,
  dispatch_callback: Callback<<T as Slice>::Action>,
}

impl<T: Slice> Clone for SliceHandle<T> {
  fn clone(&self) -> Self {
    Self {
      value: self.value.clone(),
      dispatch_callback: self.dispatch_callback.clone(),
    }
  }
}

impl<T: Slice> SliceHandle<T> {
  /// Get the current value
  pub fn value(&self) -> &T {
    &self.value
  }

  /// Dispatch an action to the reducer
  pub fn dispatch(&self, action: <T as Slice>::Action) {
    self.dispatch_callback.emit(action);
  }
}

/// Hook to get slice value and dispatcher
#[hook]
pub fn use_slice<T: Slice + 'static>() -> SliceHandle<T> {
  let states = use_context::<BounceStates>().expect("BounceRoot not found");
  use_state_subscription(states.clone());

  let value = states.get_slice_value::<T>();
  let states_clone = states.clone();
  SliceHandle {
    value,
    dispatch_callback: Callback::from(move |action: <T as Slice>::Action| {
      states_clone.dispatch_slice_action::<T>(action);
    }),
  }
}

/// Hook to get slice value only (subscribes to changes)
#[hook]
pub fn use_slice_value<T: Slice + 'static>() -> Rc<T> {
  let states = use_context::<BounceStates>().expect("BounceRoot not found");
  use_state_subscription(states.clone());
  states.get_slice_value::<T>()
}

/// Hook to get slice dispatcher only (no subscription needed)
#[hook]
pub fn use_slice_dispatch<T: Slice + 'static>() -> Callback<<T as Slice>::Action> {
  let states = use_context::<BounceStates>().expect("BounceRoot not found");

  Callback::from(move |action: <T as Slice>::Action| {
    states.dispatch_slice_action::<T>(action);
  })
}

// ---------------------------------------------------------------------------
// Selector hooks
// ---------------------------------------------------------------------------

/// Hook to get selector value with proper state management
#[hook]
pub fn use_selector_value<T: Selector + 'static>() -> Rc<T> {
  let states = use_context::<BounceStates>().expect("BounceRoot not found");
  use_state_subscription(states.clone());

  // Re-compute selector on every render triggered by state changes
  T::select(&states)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  // -- Test Atom --

  #[derive(Clone, PartialEq, Default, Debug)]
  struct Counter(i32);
  impl Atom for Counter {}

  #[test]
  fn test_atom_default_value() {
    let states = BounceStates::new();
    let value = states.get_atom_value::<Counter>();
    assert_eq!(value.0, 0, "default Counter should be 0");
  }

  #[test]
  fn test_atom_set_and_get() {
    let states = BounceStates::new();
    states.set_atom_value(Rc::new(Counter(42)));
    let value = states.get_atom_value::<Counter>();
    assert_eq!(value.0, 42);
  }

  #[test]
  fn test_atom_overwrite() {
    let states = BounceStates::new();
    states.set_atom_value(Rc::new(Counter(1)));
    states.set_atom_value(Rc::new(Counter(2)));
    let value = states.get_atom_value::<Counter>();
    assert_eq!(value.0, 2, "should hold the latest value");
  }

  // -- Test multiple atom types --

  #[derive(Clone, PartialEq, Default, Debug)]
  struct Name(String);
  impl Atom for Name {}

  #[test]
  fn test_multiple_atom_types() {
    let states = BounceStates::new();
    states.set_atom_value(Rc::new(Counter(10)));
    states.set_atom_value(Rc::new(Name("hello".into())));

    assert_eq!(states.get_atom_value::<Counter>().0, 10);
    assert_eq!(states.get_atom_value::<Name>().0, "hello");
  }

  // -- Test Slice --

  #[derive(Clone, PartialEq, Default, Debug)]
  struct TodoList(Vec<String>);

  enum TodoAction {
    Add(String),
    Clear,
  }

  impl Slice for TodoList {
    type Action = TodoAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
      match action {
        TodoAction::Add(item) => {
          let mut items = self.0.clone();
          items.push(item);
          Rc::new(TodoList(items))
        }
        TodoAction::Clear => Rc::new(TodoList(Vec::new())),
      }
    }
  }

  #[test]
  fn test_slice_default_value() {
    let states = BounceStates::new();
    let value = states.get_slice_value::<TodoList>();
    assert!(value.0.is_empty());
  }

  #[test]
  fn test_slice_dispatch() {
    let states = BounceStates::new();
    states.dispatch_slice_action::<TodoList>(TodoAction::Add("item1".into()));
    states.dispatch_slice_action::<TodoList>(TodoAction::Add("item2".into()));

    let value = states.get_slice_value::<TodoList>();
    assert_eq!(value.0, vec!["item1", "item2"]);
  }

  #[test]
  fn test_slice_clear() {
    let states = BounceStates::new();
    states.dispatch_slice_action::<TodoList>(TodoAction::Add("item1".into()));
    states.dispatch_slice_action::<TodoList>(TodoAction::Clear);

    let value = states.get_slice_value::<TodoList>();
    assert!(value.0.is_empty());
  }

  // -- Test Subscription --

  #[test]
  fn test_subscriber_notified() {
    let states = BounceStates::new();
    let call_count = Rc::new(RefCell::new(0u32));

    let count_clone = call_count.clone();
    let _sub = states.subscribe(move || {
      *count_clone.borrow_mut() += 1;
    });

    states.set_atom_value(Rc::new(Counter(1)));
    states.set_atom_value(Rc::new(Counter(2)));

    assert_eq!(*call_count.borrow(), 2, "subscriber should be called twice");
  }

  #[test]
  fn test_subscription_drop_unsubscribes() {
    let states = BounceStates::new();
    let call_count = Rc::new(RefCell::new(0u32));

    let count_clone = call_count.clone();
    let sub = states.subscribe(move || {
      *count_clone.borrow_mut() += 1;
    });

    states.set_atom_value(Rc::new(Counter(1)));
    assert_eq!(*call_count.borrow(), 1);

    // Drop subscription
    drop(sub);

    states.set_atom_value(Rc::new(Counter(2)));
    assert_eq!(
      *call_count.borrow(),
      1,
      "should not be called after unsubscribe"
    );
  }

  #[test]
  fn test_multiple_subscribers() {
    let states = BounceStates::new();
    let count_a = Rc::new(RefCell::new(0u32));
    let count_b = Rc::new(RefCell::new(0u32));

    let a = count_a.clone();
    let _sub_a = states.subscribe(move || {
      *a.borrow_mut() += 1;
    });

    let b = count_b.clone();
    let _sub_b = states.subscribe(move || {
      *b.borrow_mut() += 1;
    });

    states.set_atom_value(Rc::new(Counter(1)));

    assert_eq!(*count_a.borrow(), 1);
    assert_eq!(*count_b.borrow(), 1);
  }

  // -- Test Selector --

  #[derive(Clone, PartialEq, Default, Debug)]
  struct DoubleCounter(i32);

  impl Selector for DoubleCounter {
    fn select(states: &BounceStates) -> Rc<Self> {
      let counter = states.get_atom_value::<Counter>();
      Rc::new(DoubleCounter(counter.0 * 2))
    }
  }

  #[test]
  fn test_selector_computes_derived_state() {
    let states = BounceStates::new();
    states.set_atom_value(Rc::new(Counter(5)));

    let doubled = DoubleCounter::select(&states);
    assert_eq!(doubled.0, 10);
  }

  #[test]
  fn test_selector_recomputes_on_change() {
    let states = BounceStates::new();
    states.set_atom_value(Rc::new(Counter(3)));
    assert_eq!(DoubleCounter::select(&states).0, 6);

    states.set_atom_value(Rc::new(Counter(7)));
    assert_eq!(DoubleCounter::select(&states).0, 14);
  }
}
