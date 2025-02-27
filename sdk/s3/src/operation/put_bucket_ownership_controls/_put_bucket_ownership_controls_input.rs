// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutBucketOwnershipControlsInput {
    /// <p>The name of the Amazon S3 bucket whose <code>OwnershipControls</code> you want to set.</p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The MD5 hash of the <code>OwnershipControls</code> request body. </p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: ::std::option::Option<::std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: ::std::option::Option<::std::string::String>,
    /// <p>The <code>OwnershipControls</code> (BucketOwnerEnforced, BucketOwnerPreferred, or ObjectWriter) that you want to apply to this Amazon S3 bucket.</p>
    pub ownership_controls: ::std::option::Option<crate::types::OwnershipControls>,
}
impl PutBucketOwnershipControlsInput {
    /// <p>The name of the Amazon S3 bucket whose <code>OwnershipControls</code> you want to set.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The MD5 hash of the <code>OwnershipControls</code> request body. </p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(&self) -> ::std::option::Option<&str> {
        self.content_md5.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> ::std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
    /// <p>The <code>OwnershipControls</code> (BucketOwnerEnforced, BucketOwnerPreferred, or ObjectWriter) that you want to apply to this Amazon S3 bucket.</p>
    pub fn ownership_controls(&self) -> ::std::option::Option<&crate::types::OwnershipControls> {
        self.ownership_controls.as_ref()
    }
}
impl PutBucketOwnershipControlsInput {
    /// Creates a new builder-style object to manufacture [`PutBucketOwnershipControlsInput`](crate::operation::put_bucket_ownership_controls::PutBucketOwnershipControlsInput).
    pub fn builder() -> crate::operation::put_bucket_ownership_controls::builders::PutBucketOwnershipControlsInputBuilder {
        crate::operation::put_bucket_ownership_controls::builders::PutBucketOwnershipControlsInputBuilder::default()
    }
}

/// A builder for [`PutBucketOwnershipControlsInput`](crate::operation::put_bucket_ownership_controls::PutBucketOwnershipControlsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutBucketOwnershipControlsInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) content_md5: ::std::option::Option<::std::string::String>,
    pub(crate) expected_bucket_owner: ::std::option::Option<::std::string::String>,
    pub(crate) ownership_controls: ::std::option::Option<crate::types::OwnershipControls>,
}
impl PutBucketOwnershipControlsInputBuilder {
    /// <p>The name of the Amazon S3 bucket whose <code>OwnershipControls</code> you want to set.</p>
    /// This field is required.
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket whose <code>OwnershipControls</code> you want to set.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the Amazon S3 bucket whose <code>OwnershipControls</code> you want to set.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>The MD5 hash of the <code>OwnershipControls</code> request body. </p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content_md5 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The MD5 hash of the <code>OwnershipControls</code> request body. </p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn set_content_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content_md5 = input;
        self
    }
    /// <p>The MD5 hash of the <code>OwnershipControls</code> request body. </p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn get_content_md5(&self) -> &::std::option::Option<::std::string::String> {
        &self.content_md5
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expected_bucket_owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expected_bucket_owner = input;
        self
    }
    /// <p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn get_expected_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
        &self.expected_bucket_owner
    }
    /// <p>The <code>OwnershipControls</code> (BucketOwnerEnforced, BucketOwnerPreferred, or ObjectWriter) that you want to apply to this Amazon S3 bucket.</p>
    /// This field is required.
    pub fn ownership_controls(mut self, input: crate::types::OwnershipControls) -> Self {
        self.ownership_controls = ::std::option::Option::Some(input);
        self
    }
    /// <p>The <code>OwnershipControls</code> (BucketOwnerEnforced, BucketOwnerPreferred, or ObjectWriter) that you want to apply to this Amazon S3 bucket.</p>
    pub fn set_ownership_controls(mut self, input: ::std::option::Option<crate::types::OwnershipControls>) -> Self {
        self.ownership_controls = input;
        self
    }
    /// <p>The <code>OwnershipControls</code> (BucketOwnerEnforced, BucketOwnerPreferred, or ObjectWriter) that you want to apply to this Amazon S3 bucket.</p>
    pub fn get_ownership_controls(&self) -> &::std::option::Option<crate::types::OwnershipControls> {
        &self.ownership_controls
    }
    /// Consumes the builder and constructs a [`PutBucketOwnershipControlsInput`](crate::operation::put_bucket_ownership_controls::PutBucketOwnershipControlsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_bucket_ownership_controls::PutBucketOwnershipControlsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::put_bucket_ownership_controls::PutBucketOwnershipControlsInput {
            bucket: self.bucket,
            content_md5: self.content_md5,
            expected_bucket_owner: self.expected_bucket_owner,
            ownership_controls: self.ownership_controls,
        })
    }
}
