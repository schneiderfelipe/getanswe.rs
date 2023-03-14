# Examples

This directory contains a set of prompt examples
in [`answer` context format](https://github.com/schneiderfelipe/answer#usage).

## Act as a Linux terminal

```console
$ echo "curl https://en.wikipedia.org/api/rest_v1/page/summary/cURL" \
  | answer act-as-a-linux-terminal.yml
{
  "type": "standard",
  "title": "cURL",
  "displaytitle": "cURL",
  "extract": "cURL is a computer software project providing a library and command-line tool for transferring data using various network protocols. The name stands for \"Client URL\", which was first released in 1997. The original author and lead developer is the Swedish developer Daniel Stenberg. ..."
}
```
