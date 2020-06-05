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
let (--) n m = List.rev @@ m++n
let (--^) n m = List.rev @@ (m+1) ++ n

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

let print_array arr =
  Array.iteri (fun i a ->
      if i <> 0 then Printf.printf " ";
      Printf.printf "%d" a) arr;
  Printf.printf "\n"

let print_arrayf arr f =
  Array.iteri (fun i a ->
      if i <> 0 then Printf.printf " ";
      f a) arr;
  Printf.printf "\n"

let () =
  let area (w,d) = w*d in

  let rec solve () =
    let (n,w,d) = scan "%d %d %d" tuple3 in
    if n = 0 && w = 0 && d = 0 then ()
    else
      let a = Array.make (succ n) (0,0) in
      a.(0) <- (w, d);
      let k = ref 1 in
      ListL.iter (0 ++^ n) ~f:(fun _ ->
          let (p,s) = scan "%d %d" tuple2 in
          let p = pred p in
          let (w, d) = a.(p) in
          let s = s mod (2*(w+d)) in
          let (x, y) = if s <= w then ((s, d), (w-s, d))
            else if s <= w + d then ((w, s-w), (w, (d-(s-w))))
            else if s <= 2*w + d then ((s-(w+d), d), (w-(s-(w+d)), d))
            else ((w, s-(2*w+d)),(w, d-(s-(2*w+d))))
          in
          ListL.iter (p ++^ Array.length a) ~f:(fun i ->
              if i+1 < Array.length a then a.(i) <- a.(i+1));
          if let (<) (w0,d0) (w1,d1)=  w0*d0 < w1*d1 in x < y then
            (a.(pred !k) <- x; a.(!k) <- y;)
          else (a.(pred !k) <- y; a.(!k) <- x);
          k := succ !k);
      let b = ArrayL.map a ~f:area in
      Array.sort compare b;
      print_array b;
      solve ()
  in
  solve ()
