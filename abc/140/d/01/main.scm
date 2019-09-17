;;; File:  main.scm
;; Author: ymiyamoto
;;
;; Created on Sat Sep 14 05:05:28 2019
;;
(define (read-number)
  (string->number (read-line)))

(define (read-numbers)
  (map string->number (string-split (read-line) #\space)))

(let* ((k (cadr (read-numbers)))
       (s (string->list (read-line))))

  (define happy-num
    (+ (apply + (map (lambda (pred cur)
		       (if (and (eqv? pred #\L) (eqv? cur #\L)) 1 0))
		     s (cdr s)))
       (apply + (map (lambda (cur next)
		       (if (and (eqv? cur #\R) (eqv? next #\R)) 1 0))
		     s (cdr s)))))

  (print (min (- (length s) 1) (+ happy-num (* 2 k)))))
