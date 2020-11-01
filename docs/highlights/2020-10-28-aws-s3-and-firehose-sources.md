---
last_modified_on: "2020-10-27"
$schema: ".schema.json"
title: "New `aws_s3` and `aws_kinesis_firehose` sources"
description: "Export observability data out of AWS."
author_github: "https://github.com/binarylogic"
pr_numbers: [4101]
release: "0.11.0"
hide_on_release_notes: false
tags: ["type: announcement", "domain: sources", "platform: aws"]
---

Getting observability data out of AWS can sometimes feel like you're in a
[Rube Goldberg comic][rube_goldberg], stitching together three or more tools
and hoping it all works in the end. We want to make this easier with Vector,
and 0.11 includes our initial efforts:

1. A new [`aws_kinesis_firehose` source][aws_kinesis_firehose_source]
2. A new [`aws_cloudwatch_logs_subscription_parser` transform][aws_cloudwatch_logs_subscription_parser_transform]
3. A new [`aws_s3` source][aws_s3_Source]

## Get Started

To help you get started with these sources, we've included the following guides:

1. [Export CloudWatch logs using Vector]()
2. [Export CloudTrail logs using Vector]()
3. ...

We're eager to hear what you think about these sources! [Join our chat][chat]
and let us know.

[chat]: https://chat.vector.dev
[rube_goldberg]: https://en.wikipedia.org/wiki/Rube_Goldberg_machine