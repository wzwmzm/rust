use opencv::core::SparseMat_Hdr;
use opencv::prelude::*;
use opencv::Result;

#[test]
fn slice_detect() -> Result<()> {
	let hdr = SparseMat_Hdr::new(&[4, 2], i32::opencv_type())?;
	assert_eq!(4, hdr.size()[0]);
	assert_eq!(2, hdr.size()[1]);
	assert_eq!(0, hdr.size()[2]);
	Ok(())
}
