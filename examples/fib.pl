;; Parlance example — Fibonacci

(infix + 3)
(infix - 3)
(infix * 4)
(infix < 2)

(def fib n
  (if (< n 2)
      n
      (+ (fib (- n 1)) (fib (- n 2)))))

(fib 10)
