mod smart_pointers;
mod concurrency;

pub fn test_smart_pointers() {
  smart_pointers::box_save_to_heap();
  smart_pointers::implement_cons_list_use_box();
  smart_pointers::implement_custom_smart_pointer();
}

pub fn test_concurrency() {
  concurrency::value_move_into_closure();
  concurrency::message_passing::send_single_message();
  concurrency::message_passing::send_multiple_messages();
  concurrency::message_passing::send_messages_from_multiple_producer();
}