;;; File:  main.scm
;; Author: ymiyamoto
;;
;; Created on Mon Sep 23 02:28:14 2019
;;
(use util.match)

(define (read-number)
  (string->number (read-line)))

(define (read-numbers)
  (map string->number (string-split (read-line) #\space)))


(let* ((s (read-line)))
  (print (match s
		("Sunny" "Cloudy")
		("Cloudy" "Rainy")
		("Rainy" "Sunny"))))
