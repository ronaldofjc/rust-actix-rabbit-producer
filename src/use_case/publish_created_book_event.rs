use crosstown_bus::Bus;
use crate::entity::CreatedBookEvent;

pub fn execute(book: CreatedBookEvent) {
    let url = std::env::var("RABBIT_URL");
    let bus = Bus::new_rabbit_bus(url.unwrap());
    tracing::info!("Publish created book event, book_id: {}", book.id);
    let _event = bus.publish_event::<CreatedBookEvent>(CreatedBookEvent {
        id: book.id,
        title: book.title,
        author: book.author,
        pages: book.pages
    });
}