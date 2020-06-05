let dbg = Printf.printf "[debug]%s"

let id = fun x -> x

let scanf fmt f = Scanf.sscanf (read_line ()) fmt f

let scan_matrix n m e conv =
  let arr = Array.make_matrix n m e in
  Array.iteri (fun i line ->
      let s = Scanf.Scanning.from_string @@ read_line () in
      Array.iteri (fun j _ ->
          arr.(i).(j) <- Scanf.bscanf s " %s" conv;
        ) line) arr;
  arr

let (++) n m =
  let rec aux i =
    if i = m then [m]
    else i :: aux (i+1) in
  if n > m then [] else aux n

let (++^) n m = n ++ (m-1)

let between n x m = n <= x && x < m

let () =
  let memo = Array.make 45 0  in
  List.iter (fun i ->
      memo.(i) <-
        if i = 0 then 1
        else if i = 1 then 1
        else memo.(i-2) + memo.(i-1))
    (0++^45);
  let n =  scanf "%d" id in
  Printf.printf "%d\n" memo.(n)
