use aidee::*;

#[derive(Debug, Id, PartialEq, Eq)]
struct TestId(u32);

#[test]
#[cfg(feature = "alloc")]
fn test_vec() {
    let mut blocks = IdVec::<TestId, _>::new();

    let a = blocks.push("a");
    let b = blocks.push("b");
    let c = blocks.push("c");

    assert_eq!(blocks[a], "a");
    assert_eq!(blocks[b], "b");
    assert_eq!(blocks[c], "c");

    blocks.ids().eq((0..3).map(TestId));
    blocks.values().eq(&["a", "b", "c"]);

    assert_eq!(&blocks.values().copied().collect::<IdVec<TestId, _>>(), &blocks);
}

#[test]
fn test_slice() {
    let slice: &IdSlice<TestId, _> = IdSlice::from_slice(&[1, 2, 3]);
    assert_eq!(&slice[TestId(0)..TestId(2)], &[1, 2]);
    assert!(slice.ids().map(|id| slice[id]).eq([1, 2, 3]));
}

#[test]
#[cfg(feature = "alloc")]
fn test_bitvec() {
    let mut visited = IdBitVec::new();
    let a: TestId = visited.push(true);
    let b = visited.push(true);
    let c = visited.push(false);

    assert!(visited[a]);
    assert!(visited[b]);
    assert!(!visited[c]);

    visited.ids().eq((0..3).map(TestId));
}
