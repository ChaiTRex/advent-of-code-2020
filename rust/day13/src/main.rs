fn main() {
    let (start_time, buses) = {
        let input = std::fs::read_to_string("../13.txt").unwrap();

        let mut iter = input.lines();
        let start_time = iter.next().unwrap().parse::<u64>().unwrap();
        let buses = iter
            .next()
            .unwrap()
            .split(',')
            .map(|bus| bus.parse::<u64>().ok())
            .collect::<Vec<_>>();

        (start_time, buses)
    };

    {
        let (waiting_time, bus) = buses
            .iter()
            .flatten()
            .map(|&bus| (bus - start_time % bus, bus))
            .min()
            .unwrap();
        println!("{}", waiting_time * bus);
    }

    {
        let combined_offset = (0u64..)
            .zip(buses.iter())
            .filter_map(|(elapsed_time, bus)| bus.map(|bus| (bus - elapsed_time % bus, bus)))
            .fold(
                (0, 1),
                |(result_offset, prime_product), (item_offset, prime)| {
                    (
                        (0u64..)
                            .map(|n| n * prime_product + result_offset)
                            .find(|&n| n % prime == item_offset % prime)
                            .unwrap(),
                        prime_product * prime,
                    )
                },
            )
            .0;

        println!("{}", combined_offset);
    }
}
