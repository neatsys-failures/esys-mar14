terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  region = "ap-east-1"
}

resource "aws_instance" "app_server" {
  count = 2

  ami           = "ami-0bc44b8dc7cae9c34"
  instance_type = "t3.micro"
  key_name      = "Ephemeral"

  tags = {
    Name = "ExampleAppServerInstance - ${count.index}"
  }
}