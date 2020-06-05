let dbg = Printf.printf "[debug]%s"

let max_num = 1_000_000_000

let id = fun x -> x
let tuple2 x y = (x,y)
let tuple3 x y z = (x,y,z)
let succ x = x + 1
let pred x = x - 1

let (++) n m =
  let rec aux i =
    if i = m then [m]
    else i :: aux (i+1) in
  if n > m then [] else aux n

let (++^) n m = n ++ (m-1)

let scan fmt f = Scanf.sscanf (read_line ()) fmt f

let scan_lines n fmt f =
  List.map (fun _ -> scan fmt f) (0++^n)

let scan_matrix n m e conv =
  let arr = Array.make_matrix n m e in
  Array.iteri (fun i line ->
      let s = Scanf.Scanning.from_string @@ read_line () in
      Array.iteri (fun j _ ->
          arr.(i).(j) <- Scanf.bscanf s " %s" conv;
        ) line) arr; arr

let between n x m = n <= x && x < m

let string_to_list s =
  List.map (String.get s) (0 ++^ String.length s)

let (v,e) = scan "%d %d" tuple2
let edges =
  let a = Array.make_matrix v v None in
  scan_lines e "%d %d %d" tuple3
  |> List.iter (fun (x,y,z)-> a.(x).(y) <- Some z);
  a


let solve start =
  let memo = Array.make_matrix (1 lsl v) v None in
  let rec aux s w = match memo.(s).(w) with
    | Some va -> va
    | None ->
      if s = (1 lsl v) - 1 then
        if start = w then 0 else max_num
      else
        let va =
          List.filter (fun i -> s land (1 lsl i) = 0) (0 ++^ v)
          |> List.map (fun x ->
              match edges.(w).(x) with
              | None -> max_num
              | Some e -> e + aux (s lor (1 lsl x)) x)
          |> List.fold_left min max_num in
        memo.(s).(w) <- Some va; va in
  aux 0 start

let () =
  List.map (fun i -> solve i) (0 ++^ v)
  |> List.fold_left min max_num
  |> (fun va -> if va >= max_num then -1 else va)
  |>  Printf.printf "%d\n"
