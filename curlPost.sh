curl -k -X POST https://localhost:9231/v1/item \
 -H 'Accept: application/json' \
 -H 'Content-Type: application/json' \
 -d '{"puk":"LJ2jYptULFqYQ6bPjir+2QNMJWjgCQ0UJaSSScIxUg4=","msg":"Hello, world!","sig":"6+6/wT0lFnrsBbdxa9Jr8VPACxCxskNEx5Fft2AD+HHj0l1bxYpLwC8c+Z26ItD16Yz2n1QVem9THH0Nb8DLDg=="}' \
 > curlPost.json