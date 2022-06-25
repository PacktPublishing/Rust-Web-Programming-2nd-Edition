# this contains the dev infastructure
terraform {
  required_version = ">= 1.1.3"
}

# defines the credentials and location
provider "aws" {
    version = ">= 2.28.1"
    region = "eu-west-2"
}

# builds the EC2 instance
resource "aws_instance" "build_server" {
    ami = "ami-0fdbd8587b1cf431e"
    instance_type = "t2.medium"
    key_name = "remotebuild"
    user_data = file("server_build.sh")
    tags = {
      Name = "to-do build server"
    }
    # root disk
    root_block_device {
      volume_size           = "150"
      volume_type           = "gp2"
      # encrypted             = true
      delete_on_termination = true
    }
}

# return the IP of the server created
output "ec2_global_ips" {
  value = ["${aws_instance.build_server.*.public_ip}"]
}
