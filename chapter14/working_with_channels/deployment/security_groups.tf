# this file defines the security groups for the server and loadbalancer

resource "aws_security_group" "alb-security-group" {
    name = "to-do-LB"
    description = "the security group for the application load balancer"

    ingress {
        description = "http access"
        from_port = 80
        to_port = 80
        protocol = "tcp"
        cidr_blocks = ["0.0.0.0/0"]
    }
    ingress {
        description = "https access"
        from_port = 443
        to_port = 443
        protocol = "tcp"
        cidr_blocks = ["0.0.0.0/0"]
    }
    egress {
        from_port = 0
        to_port = 0
        protocol = "-1"
        cidr_blocks = ["0.0.0.0/0"]
    }
    tags = {
        name: "to-do-alb-sg"
    }
}


resource "aws_security_group" "webserver-security-group" {
    name = "to-do-App"
    description = "the security group for the web server"

    ingress {
        description = "http access"
        from_port = 80
        to_port = 80
        protocol = "tcp"
        security_groups = ["${aws_security_group.alb-security-group.id}"]
    }
    ingress {
        description = "SSH access"
        from_port = 22
        to_port = 22
        protocol = "tcp"
        cidr_blocks = ["0.0.0.0/0"]
    }
    egress {
        from_port = 0
        to_port = 0
        protocol = "-1"
        cidr_blocks = ["0.0.0.0/0"]
    }
    tags = {
        name: "to-do-webserver-sg"
    }
}

resource "aws_network_interface_sg_attachment" "sg_attachment_webserver" {
  count = length(aws_instance.production_server)  
  security_group_id    = aws_security_group.webserver-security-group.id
  network_interface_id = aws_instance.production_server[count.index].primary_network_interface_id
}
