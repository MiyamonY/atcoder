;;; File:  main.scm
;; Author: ymiyamoto
;;
;; Created on Mon Sep 23 02:52:35 2019
;;
(use data.heap)

(define (read-number)
  (string->number (read-line)))

(define (read-numbers)
  (map string->number (string-split (read-line) #\space)))

(define (1+ x) (+ x 1))
(define (1- x) (- x 1))

(let* ((m (cadr (read-numbers)))
       (as (read-numbers)))
  (define heap (make-binary-heap))

  (for-each
   (lambda (a) (binary-heap-push! heap a))
   as)

  (let loop ((m m))
    (if (> m 0)
	(let1 n (binary-heap-pop-max! heap)
	      (binary-heap-push! heap (div n 2))
	      (loop (1- m)))))

  (print
   (let loop ((h heap)
	      (sum 0))
     (if (binary-heap-empty? h)
	 sum
	 (loop h (+ sum (binary-heap-pop-min! h)))))))
