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

let split_on_char s c =
  Str.split (Str.regexp (String.make 1 c)) s

let () =
  let rec aux () =
    let n = scan "%d" id in
    if n <> 0 then
      begin
        let ls = scan_lines n "%s %s" tuple2 in
        let arr = Array.make (24*60*60+60*60+2) 0 in
        ListL.iter ls ~f:(fun (s0, s1) ->
            (match List.map int_of_string @@ split_on_char s0 ':' with
             | a::b::c:: _ ->arr.(60*60*a+60*b+c) <- succ arr.(60*60*a+60*b+c)
             |_ -> ());
            match List.map int_of_string @@ split_on_char s1 ':' with
            | a::b::c::_ -> arr.(60*60*a+60*b+c) <- pred arr.(60*60*a+60*b+c)
            | _ -> ());
        ListL.iter (1 ++^ (24*60*60+60*60+2)) ~f:(fun  i->
            arr.(i) <- arr.(pred i) + arr.(i));
        Array.fold_left max 0 arr |> Printf.printf "%d\n";
        aux()
      end
  in
  aux ()
