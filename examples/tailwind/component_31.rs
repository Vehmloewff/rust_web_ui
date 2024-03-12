use rust_web_ui::*;

pub struct Example31;

pub struct Example31Props {}

impl Default for Example31Props {
	fn default() -> Example31Props {
		Example31Props { }
	}
}

impl Widget<'_> for Example31 {
	type Props = Example31Props;

	fn render(mut ctx: Ctx<'_>, props: Example31Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white")]);
			props.set_attribute("style", "min-height: 712px");
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Padding(Size::Exact(32))]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("relative")]);
					props.set_attribute("x-data", "Components.popover({ open: true, focus: false })");
					props.set_attribute("x-init", "init()");
					props.set_attribute("@keydown.escape", "onEscape");
					props.set_attribute("@close-popover-group.window", "onClosePopoverGroup");
					
					props.child("1", Dynamic).run("button", |mut props| {
						props.set_attribute("type", "button");
						props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("gap-x-1"), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
						props.set_attribute("@click", "toggle");
						props.set_attribute("@mousedown", "if (open) $event.preventDefault()");
						props.set_attribute("aria-expanded", "false");
						props.set_attribute(":aria-expanded", "open.toString()");
						
						props.child("1", Dynamic).run("span", |mut props| {
							props.child("0", Label).run(|props| props.text("Solutions"));
						});
						props.child("3", Icon).run(|mut props| {
							props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
						});
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.set_attribute("x-show", "open");
						props.set_attribute("x-transition:enter", "transition ease-out duration-200");
						props.set_attribute("x-transition:enter-start", "opacity-0 translate-y-1");
						props.set_attribute("x-transition:enter-end", "opacity-100 translate-y-0");
						props.set_attribute("x-transition:leave", "transition ease-in duration-150");
						props.set_attribute("x-transition:leave-start", "opacity-100 translate-y-0");
						props.set_attribute("x-transition:leave-end", "opacity-0 translate-y-1");
						props.set_attribute("x-description", "Flyout menu, show/hide based on flyout menu state.");
						props.styles(&[Style::Noop("absolute"), Style::Noop("left-1/2"), Style::Noop("z-10"), Style::MarginTop(Size::Exact(20)), Style::Flex, Style::Noop("w-screen"), Style::Noop("max-w-max"), Style::Noop("-translate-x-1/2"), Style::PaddingX(Size::Exact(16))]);
						props.set_attribute("x-ref", "panel");
						props.set_attribute("@click.away", "open = false");
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("w-screen"), Style::Noop("max-w-md"), Style::Noop("flex-auto"), Style::Noop("overflow-hidden"), Style::Noop("rounded-3xl"), Style::Noop("bg-white"), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::Noop("shadow-lg"), Style::Noop("ring-1"), Style::Noop("ring-gray-900/5")]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Padding(Size::Exact(16))]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::Noop("gap-x-6"), Style::Noop("rounded-lg"), Style::Padding(Size::Exact(16)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Flex, Style::Width(Size::Exact(44)), Style::Width(Size::Exact(44)), Style::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Color(Color::Fg(6)), Style::NoopGroup("group-hover", &[Style::Noop("bg-white")])]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::TextColor(Color::Fg(67)), Style::NoopGroup("group-hover", &[Style::Noop("text-indigo-600")])]);
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Analytics"));
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
											});
										});
										props.child("3", Dynamic).run("p", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(4)), Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.text("Get a better understanding of your traffic"));
										});
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::Noop("gap-x-6"), Style::Noop("rounded-lg"), Style::Padding(Size::Exact(16)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Flex, Style::Width(Size::Exact(44)), Style::Width(Size::Exact(44)), Style::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Color(Color::Fg(6)), Style::NoopGroup("group-hover", &[Style::Noop("bg-white")])]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::TextColor(Color::Fg(67)), Style::NoopGroup("group-hover", &[Style::Noop("text-indigo-600")])]);
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Engagement"));
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
											});
										});
										props.child("3", Dynamic).run("p", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(4)), Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.text("Speak directly to your customers"));
										});
									});
								});
								props.child("5", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::Noop("gap-x-6"), Style::Noop("rounded-lg"), Style::Padding(Size::Exact(16)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Flex, Style::Width(Size::Exact(44)), Style::Width(Size::Exact(44)), Style::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Color(Color::Fg(6)), Style::NoopGroup("group-hover", &[Style::Noop("bg-white")])]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::TextColor(Color::Fg(67)), Style::NoopGroup("group-hover", &[Style::Noop("text-indigo-600")])]);
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Security"));
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
											});
										});
										props.child("3", Dynamic).run("p", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(4)), Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.text("Your customers' data will be safe and secure"));
										});
									});
								});
								props.child("7", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::Noop("gap-x-6"), Style::Noop("rounded-lg"), Style::Padding(Size::Exact(16)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Flex, Style::Width(Size::Exact(44)), Style::Width(Size::Exact(44)), Style::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Color(Color::Fg(6)), Style::NoopGroup("group-hover", &[Style::Noop("bg-white")])]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::TextColor(Color::Fg(67)), Style::NoopGroup("group-hover", &[Style::Noop("text-indigo-600")])]);
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Integrations"));
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
											});
										});
										props.child("3", Dynamic).run("p", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(4)), Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.text("Connect with third-party tools"));
										});
									});
								});
								props.child("9", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::Noop("gap-x-6"), Style::Noop("rounded-lg"), Style::Padding(Size::Exact(16)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Flex, Style::Width(Size::Exact(44)), Style::Width(Size::Exact(44)), Style::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Color(Color::Fg(6)), Style::NoopGroup("group-hover", &[Style::Noop("bg-white")])]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::TextColor(Color::Fg(67)), Style::NoopGroup("group-hover", &[Style::Noop("text-indigo-600")])]);
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.child("1", Dynamic).run("a", |mut props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.text("Automations"));
											props.child("1", Dynamic).run("span", |mut props| {
												props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
											});
										});
										props.child("3", Dynamic).run("p", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(4)), Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.text("Build strategic funnels that will convert"));
										});
									});
								});
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-2"), Style::Noop("divide-x"), Style::Noop("divide-gray-900/5"), Style::Color(Color::Fg(6))]);
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("gap-x-2.5"), Style::Padding(Size::Exact(12)), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(11))])]);
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::TextColor(Color::Fg(44))]);
									});
									props.child("2", Label).run(|props| props.text("Watch demo"));
								});
								props.child("3", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("gap-x-2.5"), Style::Padding(Size::Exact(12)), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(11))])]);
									
									props.child("1", Icon).run(|mut props| {
										props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::TextColor(Color::Fg(44))]);
									});
									props.child("2", Label).run(|props| props.text("Contact sales"));
								});
							});
						});
					});
				});
			});
		});
	}
}
