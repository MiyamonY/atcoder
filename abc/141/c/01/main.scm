;;; File:  main.scm
;; Author: ymiyamoto
;;
;; Created on Mon Sep 23 02:35:50 2019
;;
(define (read-number)
  (string->number (read-line)))

(define (read-numbers)
  (map string->number (string-split (read-line) #\space)))

(let*
    ((ins (read-numbers))
     (n (car ins))
     (k (cadr ins))
     (q (caddr ins))
     (as (map (lambda (_) (read-number)) (iota q)))
     (vec (make-vector (+ n 1) 0)))

  (for-each
   (lambda (a) (vector-set! vec a (+ 1 (vector-ref vec a))))
   as)

  (for-each
   (lambda (i a)
     (if (not (= i 0))
	 (print (if (> (- (+ k a) q) 0)
		    "Yes"
		    "No"))))
   (iota (+ n 1)) (vector->list vec)))
