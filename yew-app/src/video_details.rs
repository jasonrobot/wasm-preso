use crate::Video;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct VideoDetailsProps {
    pub video: Video,
}

#[function_component(VideoDetails)]
pub fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <div>{ video.view_count }</div>
        </div>
    }
}
