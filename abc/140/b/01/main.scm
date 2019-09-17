(define (read-number)
  (string->number (read-line)))

(define (read-numbers)
  (map string->number (string-split (read-line) #\space)))

(let* ((n (read-number))
       (as (list->vector (read-numbers)))
       (bs (list->vector (read-numbers)))
       (cs (list->vector (read-numbers))))

  (print (+ (fold + 0 (map (lambda (i)
			     (let1 n (vector-ref as i)
				   (vector-ref bs (- n 1))))
			   (iota n)))
	    (fold + 0 (map (lambda (i)
			     (if (= (- (vector-ref as (+ i 1)) (vector-ref as i)) 1)
				 (vector-ref cs (- (vector-ref as i) 1)) 0))
			   (drop-right (iota n) 1))))))
