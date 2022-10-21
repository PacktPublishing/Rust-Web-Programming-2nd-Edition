

data "aws_subnet_ids" "subnet" {
    vpc_id = aws_default_vpc.default.id
}

data "aws_acm_certificate" "issued_certificate" {
    domain   = "*.freshcutswags.com"
    statuses = ["ISSUED"]
}
data "aws_acm_certificate" "raw_cert" {
    domain   = "freshcutswags.com"
    statuses = ["ISSUED"]
}

resource "aws_lb_target_group" "target-group" {
    health_check {
        interval = 10
        path = "/"
        protocol = "HTTP"
        timeout = 5
        healthy_threshold = 5
        unhealthy_threshold = 2
    }

    name = "ToDoAppLbTg"
    port = 80
    protocol = "HTTP"
    target_type = "instance"
    vpc_id   = aws_default_vpc.default.id
}


resource "aws_lb" "application-lb" {
    name = "ToDoApplicationLb"
    internal = false
    ip_address_type = "ipv4"
    load_balancer_type = "application"
    security_groups = ["${aws_security_group.alb-security-group.id}"]
    subnets = data.aws_subnet_ids.subnet.ids

    tags = {
        name = "todo load balancer"
    }
}

resource "aws_lb_listener" "http-listener" {
    load_balancer_arn = aws_lb.application-lb.arn
    port = 80
    protocol = "HTTP"

    default_action {
        type = "redirect"

        redirect {
            port        = "443"
            protocol    = "HTTPS"
            status_code = "HTTP_301"
        }
    }
}

resource "aws_lb_listener" "https-listener" {
    load_balancer_arn = aws_lb.application-lb.arn
    port = 443
    protocol = "HTTPS"
    certificate_arn = data.aws_acm_certificate.issued_certificate.arn

    default_action {
        target_group_arn = aws_lb_target_group.target-group.arn
        type = "forward"
    }
}

resource "aws_lb_listener_certificate" "extra_certificate" {
  listener_arn    = "${aws_lb_listener.https-listener.arn}"
  certificate_arn = "${data.aws_acm_certificate.raw_cert.arn}"
}


resource "aws_lb_target_group_attachment" "ec2_attach" {
    count = length(aws_instance.production_server)
    target_group_arn = aws_lb_target_group.target-group.arn
    target_id = aws_instance.production_server[count.index].id
}
