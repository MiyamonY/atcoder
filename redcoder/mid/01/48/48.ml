let dbg = Printf.printf "[debug]%s"

let max_num = 1_000_000_000

let id = fun x -> x
let tuple2 x y = (x,y)
let succ x = x + 1
let pred x = x - 1

let (++) n m =
  let rec aux i =
    if i = m then [m]
    else i :: aux (i+1) in
  if n > m then [] else aux n

let (++^) n m = n ++ (m-1)

let scan fmt f = Scanf.sscanf (read_line ()) fmt f

let scan_line n =
  let b = Scanf.Scanning.from_string (read_line ()) in
  List.map (fun _ -> Scanf.bscanf b " %d" id) (0++^n)

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

let solve arr =
  let len = Array.length arr  in
  let memo = Array.make_matrix len len None in
  let rec aux n m =
    if n >= m then 0
    else
      match memo.(n).(m) with
      | Some v -> v
      | None ->
        let v =
          (let x = aux (succ n) (pred m) in
           (if x = m - n - 1
            then
              x + (if abs @@ arr.(n) - arr.(m) <= 1 then 2 else 0)
            else x))
          :: List.map (fun i ->
              aux n i + aux (succ i) m)
            (n ++^ m) |> List.fold_left max 0 in
        memo.(n).(m) <- Some v; v
  in
  aux 0 @@ pred len

let () =
  let rec aux () =
    let n = scan "%d" id in
    if n = 0 then ()
    else
      let arr = Array.of_list @@ scan_line n in
      solve arr |> Printf.printf "%d\n";
      aux () in
  aux ()
