(define (read-number)
  (string->number (read-line)))

(let* ((n (read-number)))
  (print (* n (* n n))))
