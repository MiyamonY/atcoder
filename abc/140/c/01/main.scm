(define (read-number)
  (string->number (read-line)))

(define (read-numbers)
  (map string->number (string-split (read-line)#\space)))

(let* ((n (read-number))
       (bs (read-numbers)))

  (print (+ (car bs)
	    (fold + 0 (map (lambda (x y) (min x y)) bs (drop bs 1)))
	    (last bs))))
