use rust_web_ui::*;

pub struct Example13;

pub struct Example13Props {}

impl Default for Example13Props {
	fn default() -> Example13Props {
		Example13Props { }
	}
}

impl Widget<'_> for Example13 {
	type Props = Example13Props;

	fn render(mut ctx: Ctx<'_>, props: Example13Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Color(Color::Fg(33))]);
			props.set_attribute("style", "height: 560px");
			
			props.child("1", Dynamic).run("div", |props| {
				props.set_attribute("x-data", "{ open: true }");
				props.set_attribute("@keydown.window.escape", "open = false");
				props.set_attribute("x-init", r#"$watch("open", o => !o && window.setTimeout(() => (open = true), 1000))"#);
				props.set_attribute("x-show", "open");
				props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("z-10")]);
				props.set_attribute("aria-labelledby", "modal-title");
				props.set_attribute("x-ref", "dialog");
				props.set_attribute("aria-modal", "true");
				
				props.child("1", Dynamic).run("div", |props| {
					props.set_attribute("x-show", "open");
					props.set_attribute("x-transition:enter", "ease-out duration-300");
					props.set_attribute("x-transition:enter-start", "opacity-0");
					props.set_attribute("x-transition:enter-end", "opacity-100");
					props.set_attribute("x-transition:leave", "ease-in duration-200");
					props.set_attribute("x-transition:leave-start", "opacity-100");
					props.set_attribute("x-transition:leave-end", "opacity-0");
					props.set_attribute("x-description", "Background backdrop, show/hide based on modal state.");
					props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), Style::Color(Color::Fg(56)), NoStyle::Noop("bg-opacity-75"), NoStyle::Noop("transition-opacity")]);
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-10"), NoStyle::Noop("w-screen"), NoStyle::Noop("overflow-y-auto")]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[Style::Flex, NoStyle::Noop("min-h-full"), NoStyle::Noop("items-end"), Style::JustifyCenter, Style::Padding(16), NoStyle::Noop("text-center"), Screen::Small(&[Style::ItemsCenter]), Screen::Small(&[Style::Padding(0)])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.set_attribute("x-show", "open");
							props.set_attribute("x-transition:enter", "ease-out duration-300");
							props.set_attribute("x-transition:enter-start", "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95");
							props.set_attribute("x-transition:enter-end", "opacity-100 translate-y-0 sm:scale-100");
							props.set_attribute("x-transition:leave", "ease-in duration-200");
							props.set_attribute("x-transition:leave-start", "opacity-100 translate-y-0 sm:scale-100");
							props.set_attribute("x-transition:leave-end", "opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95");
							props.set_attribute("x-description", "Modal panel, show/hide based on modal state.");
							props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("transform"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), NoStyle::Noop("bg-white"), NoStyle::Noop("text-left"), NoStyle::Noop("shadow-xl"), NoStyle::Noop("transition-all"), Screen::Small(&[Style::MarginY(32)]), Screen::Small(&[NoStyle::Noop("w-full")]), Screen::Small(&[NoStyle::Noop("max-w-lg")])]);
							props.set_attribute("@click.away", "open = false");
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("bg-white"), Style::PaddingX(16), Style::PaddingBottom(16), Style::PaddingTop(20), Screen::Small(&[Style::Padding(24)]), Screen::Small(&[Style::PaddingBottom(16)])]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Screen::Small(&[Style::Flex]), Screen::Small(&[NoStyle::Noop("items-start")])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("mx-auto"), Style::Flex, Style::Width(48), Style::Width(48), NoStyle::Noop("flex-shrink-0"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-full"), Style::Color(Color::Danger(11)), Screen::Small(&[Style::MarginX(0)]), Screen::Small(&[Style::Width(40)]), Screen::Small(&[Style::Width(40)])]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[Style::Width(24), Style::Width(24), Style::TextColor(Color::Danger(67))]);
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[Style::MarginTop(12), NoStyle::Noop("text-center"), Screen::Small(&[Style::MarginLeft(16)]), Screen::Small(&[Style::MarginTop(0)]), Screen::Small(&[NoStyle::Noop("text-left")])]);
										
										props.child("1", Dynamic).run("h3", |props| {
											props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
											props.set_attribute("id", "modal-title");
											
											props.child("0", Label).run(|props| props.set_text("Deactivate account"));
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8)]);
											
											props.child("1", Dynamic).run("p", |props| {
												props.styles(&[NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
												
												props.child("0", Label).run(|props| props.set_text("Are you sure you want to deactivate your account? All of your data will be permanently removed. This action cannot be undone."));
											});
										});
									});
								});
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[Style::Color(Color::Fg(6)), Style::PaddingX(16), Style::PaddingY(12), Screen::Small(&[Style::Flex]), Screen::Small(&[NoStyle::Noop("flex-row-reverse")]), Screen::Small(&[Style::PaddingX(24)])]);
								
								props.child("1", Dynamic).run("button", |props| {
									props.set_attribute("type", "button");
									props.styles(&[Style::InlineFlex, NoStyle::Noop("w-full"), Style::JustifyCenter, NoStyle::Noop("rounded-md"), Style::Color(Color::Danger(67)), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("text-white"), NoStyle::Noop("shadow-sm"), Action::Hover(&[Style::Color(Color::Danger(56))]), Screen::Small(&[Style::MarginLeft(12)]), Screen::Small(&[NoStyle::Noop("w-auto")])]);
									props.set_attribute("@click", "open = false");
									
									props.child("0", Label).run(|props| props.set_text("Deactivate"));
								});
								props.child("3", Dynamic).run("button", |props| {
									props.set_attribute("type", "button");
									props.styles(&[Style::MarginTop(12), Style::InlineFlex, NoStyle::Noop("w-full"), Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))]), Screen::Small(&[Style::MarginTop(0)]), Screen::Small(&[NoStyle::Noop("w-auto")])]);
									props.set_attribute("@click", "open = false");
									
									props.child("0", Label).run(|props| props.set_text("Cancel"));
								});
							});
						});
					});
				});
			});
		});
	}
}
