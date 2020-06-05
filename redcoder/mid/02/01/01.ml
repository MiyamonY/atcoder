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

let () =
  let rec aux ()=
    let (n, x) = scan "%d %d" tuple2 in
    if n = 0 && x = 0 then ()
    else
      (ListL.map (1++n) ~f:(fun i ->
           ListL.map (i+1++n) ~f:(fun j ->
               ListL.map (j+1++n) ~f:(fun k ->
                   if i + j + k = x then true
                   else false
                 )
             )
         ) |> List.concat |> List.concat |> List.filter ((=) true)
       |> List.length
       |> Printf.printf "%d\n";
       aux ())
  in aux ()
