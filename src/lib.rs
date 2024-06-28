// #opl3-rs
// A simple wrapper around the OPL3 chip library.
// Bindings generated by Daniel Balsom.
//
// Nuked OPL3 Copyright (C) 2013-2020 Nuke.YKT
#![warn(missing_docs)]
#![doc = include_str!("./docs.md")]

/*
 * Nuked OPL3 is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 2.1
 * of the License, or (at your option) any later version.
 *
 * Nuked OPL3 is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Nuked OPL3. If not, see <https://www.gnu.org/licenses/>.

 *  Nuked OPL3 emulator.
 *  Thanks:
 *      MAME Development Team(Jarek Burczynski, Tatsuyuki Satoh):
 *          Feedback and Rhythm part calculation information.
 *      forums.submarine.org.uk(carbon14, opl3):
 *          Tremolo and phase generator calculation information.
 *      OPLx decapsulated(Matthew Gambrell, Olli Niemitalo):
 *          OPL2 ROMs.
 *      siliconpr0n.org(John McMaster, digshadow):
 *          YMF262 and VRC VII decaps and die shots.
 *
 * version: 1.8
 */

mod bindings;

/// The `Opl3Chip` struct provides a safe interface for interacting with the Nuked-OPL3 library.
pub struct Opl3Chip {
    chip: bindings::Opl3Chip,
}

impl Opl3Chip {
    /// Creates a new OPL3 chip instance.
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - The sample rate to initialize the OPL3 chip with.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// ```
    pub fn new(sample_rate: u32) -> Self {
        unsafe {
            let mut chip: bindings::Opl3Chip = std::mem::zeroed();
            bindings::Opl3Reset(&mut chip, sample_rate); // Initialize with a default sample rate, for example
            Opl3Chip { chip }
        }
    }

    /// Generate audio samples.
    ///
    /// Internally, this calls Opl3Generate4Ch and returns samples for the first 2 channels..
    /// Therefore, the buffer provided must be 4 samples long.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to a buffer that will receive the audio samples.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer = [0i16; 4];
    /// chip.generate(&mut buffer);
    /// ```
    pub fn generate(&mut self, buffer: &mut [i16]) {
        if buffer.len() < 4 {
            panic!("Buffer must be at least 4 samples long.");
        }
        unsafe {
            bindings::Opl3Generate(&mut self.chip, buffer.as_mut_ptr());
        }
    }

    /// Generates resampled audio samples.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to a buffer that will be filled with resampled audio samples.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer = [0i16; 4];
    /// chip.generate_resampled(&mut buffer);
    /// ```
    pub fn generate_resampled(&mut self, buffer: &mut [i16]) {
        if buffer.len() < 4 {
            panic!("Buffer must be at least 4 samples long.");
        }
        unsafe {
            bindings::Opl3GenerateResampled(&mut self.chip, buffer.as_mut_ptr());
        }
    }

    /// Writes a value to an OPL register.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to write to.
    /// * `value` - The value to write to the register.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// chip.write_register(0x20, 0x01);
    /// ```
    pub fn write_register(&mut self, reg: u16, value: u8) {
        unsafe {
            bindings::Opl3WriteReg(&mut self.chip, reg, value);
        }
    }

    /// Write a value to an OPL register, in buffered mode.
    ///
    /// The OPL3 normally requires a delay between register writes. This function
    /// will queue the write operation and execute it after any necessary delay.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to write to.
    /// * `value` - The value to write to the register.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// chip.write_register_buffered(0x20, 0x01);
    /// ```
    pub fn write_register_buffered(&mut self, reg: u16, value: u8) {
        unsafe {
            bindings::Opl3WriteRegBuffered(&mut self.chip, reg, value);
        }
    }

    /// Generates a stream of resampled audio samples.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to a buffer that will be filled with resampled audio samples.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer = [0i16; 4];
    /// chip.generate_stream(&mut buffer);
    /// ```
    pub fn generate_stream(&mut self, buffer: &mut [i16]) {
        unsafe {
            bindings::Opl3GenerateStream(&mut self.chip, buffer.as_mut_ptr(), buffer.len() as u32);
        }
    }

    /// Generate 4 channel audio samples.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to a buffer that will receive the audio samples.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer = [0i16; 4];
    /// chip.generate_4ch(&mut buffer);
    /// ```
    pub fn generate_4ch(&mut self, buffer: &mut [i16]) {
        if buffer.len() < 4 {
            panic!("Buffer must be at least 4 samples long.");
        }
        unsafe {
            bindings::Opl3Generate4Ch(&mut self.chip, buffer.as_mut_ptr());
        }
    }

    /// Generate 4 channel resampled audio samples.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to a buffer that will receive the resampled audio samples.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer = [0i16; 4];
    /// chip.generate_4ch_resampled(&mut buffer);
    /// ```
    pub fn generate_4ch_resampled(&mut self, buffer: &mut [i16]) {
        if buffer.len() < 4 {
            panic!("Buffer must be at least 4 samples long.");
        }
        unsafe {
            bindings::Opl3Generate4ChResampled(&mut self.chip, buffer.as_mut_ptr());
        }
    }

    /// Generates a stream of 4 channel resampled audio samples.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to a buffer that will be filled with resampled audio samples.
    ///
    /// # Example
    ///
    /// ```
    /// use opl3_rs::Opl3Chip;
    ///
    /// let mut chip = Opl3Chip::new(44100);
    /// let mut buffer1 = [0i16; 4];
    /// let mut buffer2 = [0i16; 4];
    /// chip.generate_4ch_stream(&mut buffer1, &mut buffer2);
    /// ```
    pub fn generate_4ch_stream(&mut self, buffer1: &mut [i16], buffer2: &mut [i16]) {
        if buffer1.len() < 4 || buffer2.len() < 4 {
            panic!("Buffers must be at least 4 samples long.");
        }
        unsafe {
            bindings::Opl3Generate4ChStream(&mut self.chip, buffer1.as_mut_ptr(), buffer2.as_mut_ptr(), buffer1.len() as u32);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
}
