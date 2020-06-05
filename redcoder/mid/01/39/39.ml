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

let scan_array n =
  let b = Scanf.Scanning.from_string @@ read_line () in
  Array.init n (fun _ -> Scanf.bscanf b " %d" id)

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
let arr = scan_array n

let () =
  let dp = Array.make_matrix (n-1) 21 0 in
  Array.iteri (fun i a ->
      if i = 0 then
        dp.(i).(a) <- 1
      else if i = n - 1 then
        Printf.printf "%d\n" dp.(n-2).(a)
      else
        Array.iteri (fun j _ ->
            begin
              if j + a <= 20 then dp.(i).(j+a) <- dp.(i).(j+a) + dp.(i-1).(j);
              if j -a >= 0 then dp.(i).(j-a) <-dp.(i).(j-a) + dp.(i-1).(j)
            end) dp.(i))
    arr
