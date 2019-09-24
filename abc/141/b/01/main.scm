;;; File:  main.scm
;; Author: ymiyamoto
;;
;; Created on Mon Sep 23 02:29:37 2019
;;
(use util.match)

(define (read-number)
  (string->number (read-line)))

(define (read-numbers)
  (map string->number (string-split (read-line) #\space)))

(let* ((s (read-line)))
  (define result (let loop ((s (string->list s))
			    (i 1))
		   (cond ((null? s) #t)
			 ((odd? i)
			  (if (eqv? (car s) #\L)
			      #f
			      (loop (cdr s) (+ i 1))))
			 (else
			  (if (eqv? (car s) #\R)
			      #f
			      (loop (cdr s) (+ i 1)))))))

  (print (if result "Yes" "No")))
