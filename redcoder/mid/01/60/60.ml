module ArrayL = ArrayLabels
module ListL = ListLabels

let dbg = Printf.printf "[debug]%s"

let max_num = 100_000_000_000

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

let (v, e) = scan "%d %d" tuple2

let dp =
  let mat = Array.make_matrix v v max_num in
  let lines = scan_lines e "%d %d %d" tuple3 in
  ListL.iter (0++^v) ~f:(fun i ->
      mat.(i).(i) <- 0);
  ListL.iter lines ~f:(fun (s,t,d) ->
      mat.(s).(t) <- d);
  mat

let () =
  ListL.iter (0++^v) ~f:(fun k ->
      ListL.iter (0++^v) ~f:(fun i->
          ListL.iter (0++^v) ~f:(fun j->
              if dp.(i).(k) <> max_num && dp.(k).(j) <> max_num then
                dp.(i).(j) <- min dp.(i).(j) (dp.(i).(k) + dp.(k).(j))
            )
        )
    );
  if List.exists ((>) 0) @@ ListL.map (0 ++^ v) ~f:(fun i-> dp.(i).(i)) then
    Printf.printf "NEGATIVE CYCLE\n"
  else
    ListL.iter (0 ++^ v) ~f:(fun i ->
        ListL.iter (0 ++^ v) ~f:(fun j ->
            if j <> 0 then Printf.printf " ";
            Printf.printf "%s" @@
            if dp.(i).(j) <> max_num then Printf.sprintf "%d" dp.(i).(j)
            else "INF"
          );
        Printf.printf "\n")
