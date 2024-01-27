variable "localhost" {
  default     = "http://localhost"
  description = "localhost url"
}

provider "aws" {
  region                      = "us-east-1"
  access_key                  = "fake"
  secret_key                  = "fake"
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true

  endpoints {
    # apigateway      = "${var.localhost}:4566"
    # apigatewayv2    = "${var.localhost}:4566"
    # cloudformation  = "${var.localhost}:4566"
    cloudwatch = "${var.localhost}:4566"
    # cognitoidp      = "${var.localhost}:4566"
    # cognitoidentity = "${var.localhost}:4566"
    # dynamodb        = "${var.localhost}:4566"
    # ec2             = "${var.localhost}:4566"
    # es              = "${var.localhost}:4566"
    # elasticache     = "${var.localhost}:4566"
    # firehose        = "${var.localhost}:4566"
    iam = "${var.localhost}:4566"
    # kinesis         = "${var.localhost}:4566"
    lambda = "${var.localhost}:4566"
    # rds             = "${var.localhost}:4566"
    # redshift        = "${var.localhost}:4566"
    # route53         = "${var.localhost}:4566"
    # s3              = "${var.localhost}:4566"
    # secretsmanager  = "${var.localhost}:4566"
    # ses             = "${var.localhost}:4566"
    # sns             = "${var.localhost}:4566"
    # sqs             = "${var.localhost}:4566"
    # ssm             = "${var.localhost}:4566"
    # stepfunctions   = "${var.localhost}:4566"
    # sts             = "${var.localhost}:4566"
  }

  default_tags {
    tags = {
      Environment = "Local"
      Service     = "LocalStack"
    }
  }
}

terraform {
  # The configuration for this backend will be filled in by Terragrunt or via a backend.hcl file. See
  # https://www.terraform.io/docs/backends/config.html#partial-configuration
  #  backend "s3" {}

  # Only allow this Terraform version. Note that if you upgrade to a newer version, Terraform won't allow you to use an
  # older version, so when you upgrade, you should upgrade everyone on your team and your CI servers all at once.
  required_version = "= 1.5.7"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}
