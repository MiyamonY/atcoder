let dbg = Printf.printf "[debug]%s"

let id = fun x -> x
let tuple2 x y = (x,y)

let (++) n m =
  let rec aux i =
    if i = m then [m]
    else i :: aux (i+1) in
  if n > m then [] else aux n

let (++^) n m = n ++ (m-1)

let scanf fmt f = Scanf.sscanf (read_line ()) fmt f

let scanfs n fmt f =
  List.map (fun _ -> scanf fmt f) (0++^n)

let scan_array n conv =
  let b = Scanf.Scanning.from_string @@ read_line () in
  Array.init n (fun _ -> Scanf.bscanf b " %s" conv)

let scan_matrix n m e conv =
  let arr = Array.make_matrix n m e in
  Array.iteri (fun i line ->
      let s = Scanf.Scanning.from_string @@ read_line () in
      Array.iteri (fun j _ ->
          arr.(i).(j) <- Scanf.bscanf s " %s" conv;
        ) line) arr; arr

let between n x m = n <= x && x < m

let (n,m) = scanf "%d %d" tuple2
let coins = scan_array m int_of_string

let () =
  let memo = Array.make_matrix (m+1) (n+1) max_int in
  memo.(0).(0) <- 0;
  List.iter (fun i ->
      let c = coins.(i-1) in
      List.iter (fun j ->
          memo.(i).(j) <-
            if j -c >= 0 then
              let x = if memo.(i-1).(j-c) <> max_int
                then memo.(i-1).(j-c) + 1 else max_int in
              let y = if memo.(i).(j-c) <> max_int
                then memo.(i).(j-c) + 1 else max_int in
              min memo.(i-1).(j) @@ min x y
            else
              memo.(i-1).(j)
        ) (0++n)
    ) (1++m);
  Printf.printf "%d\n" memo.(m).(n)
