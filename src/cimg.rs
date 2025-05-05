use core::ptr;
use sdl2::libc::{self, c_int};
use sdl2::sys::{
    SDL_CreateTextureFromSurface, SDL_DestroyTexture, SDL_FreeSurface, SDL_LoadBMP_RW, SDL_MapRGB,
    SDL_QueryTexture, SDL_RWFromFile, SDL_Rect, SDL_RenderCopy, SDL_RenderCopyEx, SDL_Renderer,
    SDL_RendererFlip, SDL_SetColorKey, SDL_Texture,
};

pub(crate) struct CIMG {
    img: *mut SDL_Texture,
    rect: SDL_Rect,
}

impl CIMG {
    pub fn from(file_name: String, render: *mut SDL_Renderer) -> CIMG {
        let mut file_name = format!("files/images/{}.bmp", file_name);

        let bmp_rw =
            unsafe { SDL_RWFromFile(file_name.as_mut_ptr() as *mut i8, "r".as_ptr() as *const i8) };

        let surface = unsafe { SDL_LoadBMP_RW(bmp_rw, 0 as c_int) };

        let key = unsafe { SDL_MapRGB((*surface).format, 255, 0, 255) };
        unsafe {
            SDL_SetColorKey(surface, 1 as c_int, key);
        }

        let img = unsafe { SDL_CreateTextureFromSurface(render, surface) };

        let mut width = 0;
        let mut height = 0;
        unsafe {
            SDL_QueryTexture(
                img,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut width,
                &mut height,
            );
        };

        let cimg = CIMG {
            img: img,
            rect: SDL_Rect {
                x: 0,
                y: 0,
                w: width,
                h: height,
            },
        };

        unsafe {
            SDL_FreeSurface(surface);
        };

        cimg
    }

    pub fn draw(&mut self, render: *mut SDL_Renderer, x_offset: i32, y_offset: i32, rotate: bool) {
        self.rect.x = x_offset;
        self.rect.y = y_offset;

        if rotate {
            unsafe {
                SDL_RenderCopyEx(
                    render,
                    self.img,
                    ptr::null(),
                    &self.rect as *const _,
                    180.0,
                    ptr::null(),
                    SDL_RendererFlip::SDL_FLIP_VERTICAL,
                );
            }
        } else {
            unsafe {
                SDL_RenderCopy(render, self.img, ptr::null(), &self.rect as *const _);
            }
        }
    }

    pub fn draw_vert(&mut self, render: *mut SDL_Renderer, x_offset: i32, y_offset: i32) {
        self.rect.x = x_offset;
        self.rect.y = y_offset;

        unsafe {
            SDL_RenderCopyEx(
                render,
                self.img,
                ptr::null(),
                &self.rect as *const _,
                180.0,
                ptr::null(),
                SDL_RendererFlip::SDL_FLIP_HORIZONTAL,
            );
        }
    }

    pub fn draw_rect(&mut self, render: *mut SDL_Renderer, crop: SDL_Rect, rect: SDL_Rect) {
        unsafe {
            SDL_RenderCopy(render, self.img, &crop as *const _, &rect as *const _);
        }
    }
}

impl Drop for CIMG {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyTexture(self.img);
        }
    }
}
