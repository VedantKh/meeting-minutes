# Transcription Quality Troubleshooting Guide

This guide helps you achieve the best possible transcription quality with Meetily. If you're experiencing low-quality transcriptions, follow these steps to diagnose and fix the issue.

## Quick Start: Common Issues and Solutions

### Issue: Transcription quality is lower than Zoom's automatic subtitles

**Most Common Causes:**
1. **Wrong model selected** - Parakeet model provides significantly better quality than older Whisper models
2. **Audio input level too low** - Microphone volume needs adjustment
3. **Background noise** - Environment is too noisy
4. **Wrong VAD settings** - Voice Activity Detection is fragmenting speech

**Quick Fixes:**
1. Switch to **Parakeet model** (recommended) in Settings → Transcript Settings
2. Check microphone volume in System Preferences
3. Move to a quieter environment or use noise-canceling headphones
4. Adjust VAD preset for your meeting type (see below)

---

## Step 1: Choose the Right Transcription Model

### Recommended: Parakeet Model ⚡

**Why Parakeet?**
- **Real-time performance**: Processes audio as you speak
- **Higher accuracy**: Especially for conversational speech
- **Better handling of accents**: More robust to different speaking styles
- **Lower resource usage**: Runs efficiently on most hardware

**How to switch to Parakeet:**
1. Open Meetily Settings
2. Go to "Transcript Settings"
3. Select "⚡ Parakeet (Recommended - Real-time / Accurate)"
4. Download the recommended model: `parakeet-tdt-0.6b-v3-int8`
5. Wait for download to complete (first time only)

### Alternative: Whisper Models

If Parakeet doesn't work for your use case, Whisper models are available:

| Model | Size | Quality | Speed | Best For |
|-------|------|---------|-------|----------|
| `large-v3` | 3GB | Highest | Slowest | Maximum accuracy, powerful hardware |
| `medium` | 1.5GB | High | Moderate | Good balance of quality and speed |
| `small` | 500MB | Good | Fast | Quick transcription, lower-end hardware |

**Note:** Larger Whisper models don't always mean better quality for conversational speech. Parakeet often outperforms even `large-v3` for meetings.

---

## Step 2: Verify Audio Input Quality

### Check Microphone Levels

**macOS:**
1. Open System Preferences → Sound → Input
2. Speak normally and watch the input level meter
3. Adjust "Input volume" so the meter reaches 50-75% during normal speech
4. Avoid levels that constantly hit 100% (causes distortion)

**Windows:**
1. Right-click speaker icon → Sounds → Recording tab
2. Select your microphone → Properties → Levels
3. Adjust microphone level to 70-80%
4. Test by speaking and watching the level indicator

### Audio Quality Indicators

Meetily monitors audio quality in real-time. Check the logs for warnings:

**Good Audio:**
```
✅ Audio quality is good
   RMS Level: 0.15 (-16.5 dB)
   Peak Level: 0.45 (-6.9 dB)
   SNR: 25.3 dB
```

**Problem Audio:**
```
⚠️ Audio level too low. Increase microphone volume or speak closer to mic.
   RMS Level: 0.005 (-46.0 dB)
```

```
⚠️ Audio clipping detected! Reduce microphone gain or move away from mic.
   Peak Level: 1.00 (0.0 dB)
```

```
⚠️ High background noise detected. Use a quieter environment or noise-canceling microphone.
   SNR: 8.2 dB
```

---

## Step 3: Optimize VAD (Voice Activity Detection) Settings

VAD determines when speech starts and stops. Wrong settings can fragment your speech into tiny segments or miss parts of conversations.

### Choose the Right VAD Preset

**One-on-One (Default)**
- Best for: 1:1 conversations, interviews
- Characteristics: Moderate sensitivity, 2-second pause bridging
- Use when: Two people taking turns speaking

**Group Meeting**
- Best for: Team meetings, discussions with 3+ people
- Characteristics: More lenient, 3-second pause bridging
- Use when: Multiple speakers, overlapping conversations

**Presentation**
- Best for: Lectures, presentations, long monologues
- Characteristics: Very lenient, 4-second pause bridging
- Use when: One person speaking continuously with thinking pauses

**Noisy Environment**
- Best for: Cafes, open offices, busy environments
- Characteristics: Higher thresholds to filter background noise
- Use when: Significant background noise is present

### How to Change VAD Preset

Currently, VAD presets are configured in the code. Future versions will expose this in the UI. For now, the default "One-on-One" preset works well for most use cases.

---

## Step 4: Improve Audio Capture Setup

### Microphone Selection

**Best Options:**
1. **USB condenser microphone** (e.g., Blue Yeti, Audio-Technica AT2020USB+)
2. **Headset with boom microphone** (e.g., HyperX Cloud, SteelSeries Arctis)
3. **Wireless headphones with good mics** (e.g., AirPods Pro, Sony WH-1000XM5)

**Avoid:**
- Built-in laptop microphones (too far from mouth, picks up keyboard noise)
- Low-quality USB microphones (high noise floor)
- Bluetooth speakers as microphones (poor audio quality)

### Microphone Positioning

**Optimal Setup:**
- **Distance**: 6-12 inches (15-30 cm) from your mouth
- **Angle**: Slightly off-axis (not directly in front) to reduce plosives (p, b, t sounds)
- **Height**: At mouth level, not below or above

**Common Mistakes:**
- Too far away (> 2 feet) - results in low volume and more room noise
- Too close (< 3 inches) - causes distortion and plosives
- Behind laptop screen - muffled sound

### Environment Optimization

**Ideal Environment:**
- Quiet room with minimal background noise
- Soft furnishings (curtains, carpet, furniture) to reduce echo
- Closed windows to block outside noise
- Turn off fans, AC, or other noise sources during recording

**Quick Fixes:**
- Use a closet or room with lots of clothes (natural sound dampening)
- Add blankets or pillows around recording area
- Use noise-canceling headphones to reduce feedback

---

## Step 5: Troubleshoot Specific Issues

### Issue: Transcription is cutting off mid-sentence

**Cause:** VAD is too aggressive or audio level is inconsistent

**Solutions:**
1. Switch to "Group Meeting" or "Presentation" VAD preset (more lenient)
2. Increase microphone volume to maintain consistent level
3. Speak at a more consistent volume
4. Check for microphone auto-gain control (AGC) and disable it

### Issue: Transcription includes lots of "[inaudible]" or blank sections

**Cause:** Audio quality too low or VAD filtering out speech

**Solutions:**
1. Increase microphone input level
2. Move closer to microphone
3. Check that correct microphone is selected in Meetily
4. Disable noise suppression if it's too aggressive
5. Test microphone in another app to verify it's working

### Issue: Transcription has many errors or wrong words

**Cause:** Model not suitable for your accent/language or audio quality issues

**Solutions:**
1. Switch to Parakeet model (better accent handling)
2. If using Whisper, try `large-v3` model for better accuracy
3. Ensure language is set correctly in settings
4. Improve audio quality (see Step 2)
5. Speak more clearly and at moderate pace

### Issue: Transcription is delayed or slow

**Cause:** Model too large for your hardware or system resources low

**Solutions:**
1. Use Parakeet model (faster than Whisper)
2. If using Whisper, switch to `small` or `medium` model
3. Close other resource-intensive applications
4. Check CPU/GPU usage during transcription
5. Ensure GPU acceleration is enabled (if available)

### Issue: Bluetooth microphone quality is poor

**Cause:** Bluetooth audio compression and latency

**Solutions:**
1. Use wired connection if possible (USB or 3.5mm)
2. Ensure Bluetooth device is in "headset" mode, not "audio" mode
3. Keep Bluetooth device close to computer (< 10 feet)
4. Check for Bluetooth interference (other devices, WiFi)
5. Update Bluetooth drivers

### Issue: System audio (Zoom, Teams) not being captured

**Cause:** System audio permissions or device selection

**Solutions:**
1. Grant system audio permissions in macOS System Preferences
2. Select correct system audio device in Meetily
3. On macOS, install BlackHole or similar virtual audio device
4. Test system audio capture before meeting
5. Check that application audio is not muted

---

## Step 6: Advanced Optimization

### Hardware Acceleration

**GPU Acceleration:**
- **macOS**: Automatically uses Apple Silicon (Metal) + CoreML
- **Windows/Linux**: Uses NVIDIA CUDA, AMD/Intel Vulkan if available
- **Benefit**: 2-5x faster transcription, lower CPU usage

**Check if GPU is being used:**
- Look for "GPU acceleration enabled" in logs
- Monitor GPU usage during transcription
- If not enabled, check GPU drivers are up to date

### Audio Processing Pipeline

Meetily applies several audio enhancements:

1. **High-pass filter** (80 Hz) - Removes low-frequency rumble
2. **RNNoise** (optional) - AI-based noise suppression
3. **EBU R128 normalization** - Standardizes loudness to -23 LUFS
4. **Resampling** - Converts to 48kHz if needed
5. **VAD** - Detects and extracts speech segments

**Note:** These are automatically applied. If transcription quality is good, don't change these settings.

### Custom VAD Tuning (Advanced)

For advanced users who want to fine-tune VAD:

**Parameters:**
- `positive_threshold` (0.0-1.0): Confidence needed to start speech detection (higher = more strict)
- `negative_threshold` (0.0-1.0): Confidence needed to end speech detection (lower = more strict)
- `redemption_time` (ms): How long to bridge short pauses (higher = fewer fragments)
- `min_speech_time` (ms): Minimum speech duration to keep (higher = filters short noises)

**Example Custom Settings:**
```rust
// Very lenient for continuous speech
positive_threshold: 0.40
negative_threshold: 0.25
redemption_time: 4000ms
min_speech_time: 400ms
```

---

## Comparison with Zoom/Teams Transcription

### Why might Zoom seem better?

1. **Cloud processing**: Zoom uses powerful cloud servers, not your local machine
2. **Proprietary models**: Zoom has custom-trained models optimized for video calls
3. **Network audio**: Zoom receives compressed audio, which may have different characteristics
4. **Post-processing**: Zoom applies heavy post-processing and error correction

### Why Meetily can be better:

1. **Privacy**: All processing happens locally, no data sent to cloud
2. **Customization**: You control the model, settings, and processing
3. **No internet required**: Works offline
4. **Better for sensitive content**: No risk of data leaks
5. **With proper setup**: Can match or exceed Zoom quality

**Key Point:** Meetily requires proper setup (right model, good audio, correct settings) to match cloud services. Once configured correctly, quality is comparable or better.

---

## Getting Help

If you've followed this guide and still have quality issues:

1. **Check GitHub Issues**: https://github.com/Zackriya-Solutions/meeting-minutes/issues
2. **Discord Community**: https://discord.gg/crRymMQBFH
3. **Include in your report:**
   - Model being used (Parakeet or Whisper version)
   - Operating system and version
   - Microphone type and model
   - Audio quality metrics from logs
   - Example of poor transcription vs expected result

---

## Quick Reference: Quality Checklist

Before each important meeting, verify:

- [ ] **Parakeet model** is selected and downloaded
- [ ] **Microphone level** is 50-75% during normal speech
- [ ] **Correct microphone** is selected in Meetily
- [ ] **Environment is quiet** (close windows, turn off fans)
- [ ] **Microphone position** is 6-12 inches from mouth
- [ ] **VAD preset** matches your meeting type
- [ ] **System audio** is configured (if capturing Zoom/Teams)
- [ ] **Test recording** works before meeting starts

---

## Version History

- **v0.1.1**: Initial quality improvements (VAD tuning, resampling fixes)
- **v0.1.2**: Added Parakeet model support, quality diagnostics
- **Current**: VAD presets, comprehensive troubleshooting guide

---

*This guide addresses GitHub Issue #171: Quality of meeting minutes*
*Last updated: 2025-10-24*
