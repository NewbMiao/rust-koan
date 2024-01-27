resource "aws_lambda_function" "lambda_demo" {
  filename         = "bootstrap.zip"
  function_name    = "lambda_demo"
  role             = aws_iam_role.iam_for_lambda_tf.arn
  handler          = "bootstrap"
  source_code_hash = filebase64sha256("bootstrap.zip")
  runtime          = "provided.al2"
  architectures    = ["arm64"]
}

resource "aws_lambda_function_url" "lambda_demo_url" {
  function_name      = aws_lambda_function.lambda_demo.arn
  authorization_type = "NONE"
}

output "function_url" {
  description = "Function URL."
  value       = aws_lambda_function_url.lambda_demo_url.function_url
}

resource "aws_iam_role" "iam_for_lambda_tf" {
  name = "iam_for_lambda_tf"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF
}
