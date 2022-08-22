dev:
	trunk serve
build:
	trunk build --release
publish:
	 az storage blob upload-batch -s dist -d '$web' --account-name deversifyadmin
