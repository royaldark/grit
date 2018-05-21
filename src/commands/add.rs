use commands::update_index;
use helpers;

pub fn add() {
    helpers::assert_in_repo();

    for file in vec![] {
        update_index::update_index(file, update_index::UpdateIndexOpts { add: true });
    }
}
