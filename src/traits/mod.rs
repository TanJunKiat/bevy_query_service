// =========================================================================
/*
 * Copyright (C) 2019 Tan Jun Kiat
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/
// =========================================================================
use super::*;

pub trait QueryFeedbackOpsSingle<T> {
    fn is_completed(&self) -> bool;
    fn increment_feedback(&mut self) -> T;
}

pub trait QueryReplyOpsSingle<T> {
    fn get_reply(request: &QueryRequest<T>) -> Self;
}

pub trait QueryReplyOpsDouble<T, U>: Sized
where
    U: Component,
{
    fn get_reply(request: &QueryRequest<T>, supplementary_queries: &Query<&U, With<U>>) -> Result<Self,()>;
}

pub trait QueryReplyOpsTriple<T, U, V>: Sized
where
    U: Component,
    V: Component,
{
    fn get_reply(request: &QueryRequest<T>, parent_queries: &Query<(Entity, &U), With<U>>, child_queries: &Query<(&bevy_hierarchy::Parent, Entity, &V), With<V>>) -> Result<Self,()>;
}
