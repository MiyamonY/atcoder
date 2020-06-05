let dbg = Printf.printf "[debug]%s"

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

let string_to_list s =
  List.map (String.get s) (0 ++^ String.length s)

let n = scanf "%d" id

let () =
  List.iter
    (fun _ ->
       let a = string_to_list @@ scanf "%s" id in
       let b = string_to_list @@ scanf "%s" id in
       let arr = Array.make_matrix (List.length a) (List.length b) 0 in
       List.iteri (fun i a ->
           List.iteri (fun j b ->
               arr.(i).(j) <-
                 if a = b then
                   1 + if i > 0 && j > 0 then arr.(i-1).(j-1) else 0
                 else
                   max (if i > 0 then arr.(i-1).(j) else 0)
                     (if j > 0 then arr.(i).(j-1) else 0)
             ) b
         ) a;
       Printf.printf "%d\n"
         arr.(pred @@ List.length a).(pred @@ List.length b)
    ) (0 ++^ n)
