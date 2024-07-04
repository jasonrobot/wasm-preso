use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub view_count: u64,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    html! {
        <ul>
            {
                videos.iter().map(|video| html! {
                    <li key={video.id}>
                      {format!("{} ({} views)", video.title, video.view_count)}
                    </li>
                }).collect::<Html>()
            }
        </ul>
    }
}
