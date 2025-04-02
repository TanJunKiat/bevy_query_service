# bevy_query_service

`bevy_query_service` is a bevy library that aims to help with handling requests that requires an input and expects an output (like HTTP calls).

The motivation comes from the limitation of `events` in `Bevy`, where events are one-way triggers that leaves the user hanging, whether or not the event is successfully triggered and what's the outcome of the event.

# Usage
To use `bevy_query_service`, simply add it to your `Cargo.toml` or run `cargo add bevy_query_service`.

To add a new request, you need a few elements:
## Request and reply component
The request and reply are defined as components, to be bundled with the entity.
```rust
#[derive(Component, Clone)]
struct Request
{
    /* … */
}

#[derive(Component, Clone)]
struct Reply
{
    /* … */
}
```
## Request event wrapper
An `QueryEvent` needs to be added to the system for the request to go through.
```rust
app.add_event::<QueryEvent<Request>>();
```
## Request server
To generate a reply, a `run_query_server` needs to be added.
```rust
app.add_systems(Update, run_query_server::<Request, Reply>);
```

To calculate the reply, a `QueryReplyOps` trait need to be defined.
```rust
impl QueryReplyOps<Request> for Reply {
    fn get_reply(world: &mut World, request: &QueryRequest<Request>) -> Self {
        /* … */
    }
}
```
The `get_reply()` function has access to the `World`, which allows easy access to all the entities and resources in the application. This should help users to get all the information they need to formulate a reply.

# Methodology
The methodology of `bevy_query_service` is to use entities to store the requests and process them in the application, before marking them as compeleted.

In each entity, there is a `GoalComponent`:
```rust
#[derive(Component, Debug, Clone, Default)]
pub struct GoalComponent {
    pub uuid: uuid::Uuid,
    pub is_completed: bool,
    pub to_delete: bool,
    pub timer: bevy_time::Stopwatch,
}
```
The `uuid` variable allows any system in the application to send a request and retrive the reply using the `query` function of `Bevy`.

`is_completed` helps to keep track if the reply component is constructed while `to_delete` marks the entity to be deleted by the optional `cleanup_requests` system. The cleaning up action is kept optional because users might want to implement their own cleanup systems.

The `timer` component is a work-in-progress. The aim is to have a timer that automatically drops the request after a certain timeout.

# Features
- [ ] options of feedback