# sylphid
A library/tool to extract dominant colors from image.

Provide a C API dynamic library , a rust library => Is ready.

And a GUI tool => Current work in progress.

## Compile
```bash
git clone https://github.com/chinuno-usami/sylphid.git
cd sylphid
cargo build --release
```

Targets output in `./target/release/`.

## libsylphid
### Example in C/C++
* Declare interface
```C++
struct Sylphid;
extern "C"{
	Sylphid* fie_create();
	unsigned long fie_result_size(Sylphid *p);
	unsigned long fie_result_at(Sylphid *p, unsigned long idx);
	void fie_load_from_raw(Sylphid *p, unsigned long width, unsigned long height, unsigned char* buff);
	void fie_load_from_file(Sylphid *p, const char path[]);
	unsigned char fie_loaded(Sylphid *p);
	void fie_run(Sylphid* p,unsigned long num, unsigned long iter_time, unsigned long dist);
	void fie_destroy(Sylphid* p);
}
```

* Create handle with `Sylphid* p = fie_create();`

* Load image from raw data (example in OpenCV)
```C++
Mat image;
image = imread( path, 1 );
fie_load_from_raw(p,image.cols,image.rows,image.data);
```

* Or load image by file path by `fie_load_from_file(p,path)`

* Check is image loaded by `fie_loaded(p)`

* Simply run with `fie_run(p,5,5000,1);`

* And finally.Output the result
```C++
for (unsigned long  i = 0; i < fie_result_size(p); ++i) {
    unsigned long res = fie_result_at(p,i);
    cout << i <<':';
    for(int i=2; i>=0; --i){
        cout << (res>>(i*8)&0xff)<<',';
    }
    cout <<endl;
}
```

* Don't forget to release the resouce by `fie_destroy(p)`

### For other languages
Check the FFI document of your language.

## fie-gui
Work in progress.
