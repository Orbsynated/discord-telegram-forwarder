use log::{error, log_enabled};
use std::fmt::Debug;

pub trait ErrorExtensions<T> {
	fn expect_with_log(self, msg: &str) -> T;
}

impl<T, E> ErrorExtensions<T> for Result<T, E>
where
	E: Debug,
{
	#[inline]
	#[track_caller]
	fn expect_with_log(self, msg: &str) -> T {
		if log_enabled!(log::Level::Error) {
			match self {
				Ok(d) => d,
				Err(e) => {
					error!("{} {:#?}", msg, e);
					unwrap_failed(msg, &e)
				}
			}
		} else {
			return self.expect(msg);
		}
	}
}

#[inline(never)]
#[cold]
#[track_caller]
fn unwrap_failed(msg: &str, error: &dyn Debug) -> ! {
	panic!("{}: {:?}", msg, error)
}
