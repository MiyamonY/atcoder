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

let solve n _ codes samples =
  let dp = Array.make_matrix (n+1) 256 max_int in
  dp.(0).(128) <- 0;
  List.iteri (fun i s ->
      List.iter (fun j ->
          List.iter (fun c ->
              let m = if j+c > 255 then 255
                else if j+c < 0 then 0
                else j+c in
              dp.(i+1).(m) <-
                min dp.(i+1).(m) (if dp.(i).(j) <> max_int then dp.(i).(j) + (m-s) *(m-s) else max_int)
            ) codes
        ) (0 ++ 255)
    ) samples;
  Array.fold_left min max_int dp.(n)

let () =
  let rec aux () =
    let (n, m) = scan "%d %d" tuple2 in
    if n = 0 && m = 0 then ()
    else
      begin
        let codes = scan_lines m "%d" id in
        let samples = scan_lines n "%d" id in
        Printf.printf "%d\n" @@ solve n m codes samples;
        aux ()
      end
  in
  aux ()
