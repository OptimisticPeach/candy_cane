use candy_cane::RawCandyCane;
use candy_cane::CandyCane;
use candy_cane::CandyCaneWriteGuard;

use candy_cane::iter::streaming::RawCandyCaneIterStreaming;
use candy_cane::iter::streaming::CandyCaneIterStreamingMut;
use candy_cane::iter::streaming::CandyCaneIterStreaming;

use parking_lot::RawRwLock;

#[test]
fn everything_is_accessible() {
    let cane: RawCandyCane<_, _, 6> = CandyCane::<()>::new();
    let _: CandyCaneWriteGuard<_, _, 6> = cane.write();

    let _: RawCandyCaneIterStreaming<'_, RawRwLock, ()>;
    let _: CandyCaneIterStreaming<_, _> = cane.iter_streaming(..);
    let _: CandyCaneIterStreamingMut<_, _> = cane.iter_streaming_mut(..);
}
