//! Utils for commitment calculation.

// Importing necessary modules and items.
use zkevm_test_harness::witness::utils::{
    events_queue_commitment_fixed, initial_heap_content_commitment_fixed,
};
use zksync_types::{LogQuery, H256, U256, USED_BOOTLOADER_MEMORY_BYTES};
use zksync_utils::expand_memory_contents;

// Calculate commitment for events queue.
pub fn events_queue_commitment(events_queue: &Vec<LogQuery>, is_pre_boojum: bool) -> Option<H256> {
    // Calculate the commitment only if it's not a pre-Boojum context.
    (!is_pre_boojum).then(|| H256(events_queue_commitment_fixed(events_queue)))
}

// Calculate commitment for bootloader's initial content.
pub fn bootloader_initial_content_commitment(
    initial_bootloader_contents: &[(usize, U256)],
    is_pre_boojum: bool,
) -> Option<H256> {
    // Calculate the commitment only if it's not a pre-

