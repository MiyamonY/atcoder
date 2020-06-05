module ArrayL = ArrayLabels
module ListL = ListLabels

let dbg = Printf.printf "[debug]%s"

let max_num = 100_000_000_000

let id = fun x -> x
let tuple2 x y = (x,y)
let tuple2_first (x, _) = x
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

let find_sec a =
  let acc = ListL.fold_left
      (List.combine (0++^ List.length a) a) ~init:[] ~f:(fun pred (i, w) ->
          match pred with
          | [] -> (w, 1, i)::pred
          | (v, n, i) :: rest when v = w -> (v, n+1, i)::rest
          | _ :: _ -> (w,1, i)::pred) in
  try ListL.find acc ~f:(fun (_, n,_) -> n >= 3)
  with Not_found -> (0, 0, 0)

let rec create n=
  if n = 0 then []
  else 0::create (pred n)

let print arr =
  Array.iter (fun v ->
      Printf.printf "%d " v ) arr;
  Printf.printf "\n"

let print_mat mat =
  Array.iter print mat

let () =
  let rec solve ()=
    let n = scan "%d" id in
    if n = 0 then ()
    else
      let mat = scan_matrix n 5 0 int_of_string in
      ListL.map (0 ++ (succ n)) ~f:(fun _ ->
          let v = ArrayL.mapi mat ~f:(fun i a ->
              let l = Array.to_list a in
              let (w, n, k) = find_sec l in
              if w > 0 then
                ListL.map (0 ++^ n) ~f:(fun j ->
                    mat.(i).(k+j) <- 0;w) |> List.fold_left (+) 0
              else 0) |> Array.fold_left (+) 0 in
          ListL.iter (0++^5) ~f:(fun j ->
              let a =
                let l = ListL.filter
                    (ListL.map (0 ++^ n) ~f:(fun i -> mat.(i).(j))) ~f:((<>) 0) in
                create (n - List.length l) @ l
                |> Array.of_list in
              ListL.iter (0 ++^ n) ~f:(fun i ->
                  mat.(i).(j) <- a.(i)));
          v)
      |> List.fold_left (+) 0
      |> Printf.printf "%d\n";
      solve () in
  solve ()
