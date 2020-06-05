let dbg = Printf.printf "[debug]%s"

let id = fun x -> x
let tuple2 x y = (x,y)
let succ x = x + 1
let pred x = x - 1

let (++) n m =
  let rec aux i acc =
    if i = m then List.rev (m::acc)
    else aux (i+1) (i::acc) in
  if n > m then [] else aux n []

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

let m = List.map (fun i -> i*(i+1)*(i+2)/6) (1++200)

let max_val = 1_000_000

let dp = Array.init (max_val+1) (fun i -> if i = 0 then 0 else max_int)
let dp_odd = Array.init (max_val+1) (fun i -> if i = 0 then 0 else max_int);;

List.iter (fun i ->
    dp.(i) <-
      List.fold_left (fun acc num ->
          min acc
            (if i >= num then succ dp.(i-num) else max_int)) max_int m)
  (1++max_val);;

let m = List.filter (fun x -> x mod 2 = 1) m in
List.iter (fun i ->
    dp_odd.(i) <-
      List.fold_left (fun acc num ->
          min acc
            (if i >= num then succ dp_odd.(i-num) else max_int)) max_int m)
  (1++max_val)

let () =
  let rec aux () =
    let n = scan "%d" id in
    if n <> 0 then
      begin
        Printf.printf "%d %d\n" dp.(n) dp_odd.(n);
        aux ()
      end
  in
  aux ()
