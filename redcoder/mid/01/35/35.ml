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

let scan_matrix n m e conv =
  let arr = Array.make_matrix n m e in
  Array.iteri (fun i line ->
      let s = Scanf.Scanning.from_string @@ read_line () in
      Array.iteri (fun j _ ->
          arr.(i).(j) <- Scanf.bscanf s " %s" conv;
        ) line) arr; arr

let between n x m = n <= x && x < m

let (n, w) = scanf "%d %d" tuple2
let items = Array.of_list @@ scanfs n "%d %d" tuple2
let memo = Array.make_matrix (n+1) (w+1) (-1)

let () =
  memo.(0).(0) <- 0;
  Array.iteri (fun i (v0, w0) ->
      List.iter (fun j ->
          if j - w0 >= 0 && memo.(i).(j-w0) >= 0 then
            memo.(i+1).(j) <- max memo.(i).(j) (memo.(i).(j-w0) + v0)
          else
            memo.(i+1).(j) <- memo.(i).(j)
        ) (0++w)) items;
  Array.fold_left max 0 memo.(n)
  |> Printf.printf "%d\n"
