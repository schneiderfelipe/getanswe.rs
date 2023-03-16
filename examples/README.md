# Examples

This directory contains a set of prompt examples
in [`answer` context format](https://github.com/schneiderfelipe/getanswer#usage).

## [Awesome ChatGPT Prompts](https://github.com/f/awesome-chatgpt-prompts)

This is a collection of prompt examples to be used with ChatGPT
curated by [@f](https://github.com/f).
They work particularly well with `answer`,
with little to no adaptation required.

For example,
the
["act as a Linux terminal" example](https://github.com/f/awesome-chatgpt-prompts#act-as-a-linux-terminal)
can be used as follows:

```yaml
# act-as-a-linux-terminal.yml
messages:
  - role: system
    content: >-
      I want you to act as a linux terminal.
      I will type commands
      and you will reply with what the terminal should show.
      I want you to only reply
      with the terminal output inside one unique code block,
      and nothing else.
      do not write explanations.
      do not type commands unless I instruct you to do so.
      When I need to tell you something in English,
      I will do so by putting text
      inside curly brackets {like this}.
```

```console
$ echo 'fortune | cowsay' | answer act-as-a-linux-terminal.yml
 _________________________________________
/ You will be successful in your work.     \
|                                         |
\ Cow says: Moooooooove towards success!   /
 -----------------------------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```
