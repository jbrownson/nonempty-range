# NonemptyRange

I needed a type representing a Nonempty range and couldn't find one, so I thought I'd make one and publish a crate. To be super thorough I'd need to make a complete copy of the other Range types but nonempty. Feel free to submit a PR :). I tried generalizing the usize, but it wound up requiring the unstable Step trait, and it becomes possible to represent negative `size_minus_one`s so I decided to leave it at usize, but I'm interested in generalizing.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags).

## Authors

* **Jake Brownson** - *Initial work*

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details